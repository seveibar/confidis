# Confidis

Confidis is a key store for uncertain answers from multiple sources with mixed reliability.

If you're aggregating information from multiple sources with mixed reliability, confidis will help you find the truth.

More technically, confidis is a truth discovery engine that interatively updates a graph of bayesian probabilities and
is tuned for adverserial scenarios.

## Features

* Simple, easy-to-use API
* Determine the quality of sources and the probability of their correctness
* Determine the probability that an answer is correct
* Performant against thousands of sources and millions of questions
* Can determine truth with exclusively poor sources (independent sources with accuracies less than 50%) 
* Tested and adaptable to [adverserial scenarios](https://github.com/waoai/confidis/blob/master/ATTACKS.md)
* Compare complex, multivariate answers

## Terms

- question: An uncertain key.
- source: An entity, e.g. a person, who can supply answers
- answer: An answer to a question from a source. An uncertain value.
- comparator: A way of comparing answers. If a comparator returns `0`, that means that two answers are equal. If a comparator returns `1` or greater, than means the answers are different. If a comparator returns `0...1`, that means that the answers are in some degree of agreement.
- distribution: Each question is part of a distribution which is specified as a prefix in the question id in the form `<distribution_id>.*`. A distribution defines some properties about the agreement of questions, e.g., the likelihood of guessing correctly.

## API

### Simple Query API

```bash
SET 
ADD QUESTION <question_id> <question_content> <question_type>

GET ANSWER <question_id>
# Returns { "confidence": 0.88, "answer": 123, "correctSources": Array<source_id> }

ADD ANSWER <answer_content> FOR <question_id> FROM <source_id>

ADD COMPARATOR <question_type> <builtin_comparator | tcp://zmq_socket_addr | http://mycompareendpoint.com/compare>

# Other commands
CLEAR ALL QUESTIONS
CLEAR ALL ANSWERS
REMOVE ANSWER TO <question_id> FROM <source_id>
REMOVE QUESTION <question_id>
GET DISTRIBUTION STATS <distribution_id>
```

### LTM vs SimpleScore

SimpleScore is a fast probabilistic algorithm that iteratively computes the confidence
and quality of each question/source whenever something is SET. There are edges cases
where SimpleScore is not accurate. SimpleScore is the default.

A Latent Truth Model is much more accurate, but takes longer to compute. For applications
where precise confidence and accuracy is needed LTM mode should be preferred. LTMs don't
perform well on several adverserial attacks (specifically, "bad sources")
