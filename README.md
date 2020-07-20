# Confidis

Confidis is a key store for uncertain answers from multiple arguing sources with unknown reliability.

[try it online!](https://waoai.github.io/confidis/) or [watch me explain a simple example](https://www.youtube.com/watch?v=QoiWacSQErg)

If you're aggregating information from multiple sources with mixed reliability, confidis will help you find the truth.

More technically, confidis is a Rust-WebAssembly truth discovery engine that interatively updates a graph of bayesian probabilities and
is tuned for adverserial scenarios.

## Features

* Simple, easy-to-use API
* Determine the quality of sources and the probability of their correctness
* Determine the probability that an answer is correct
* Performant against thousands of sources and millions of questions
* Can determine truth with exclusively poor sources (independent sources with accuracies less than 50%) 
* Tested and adaptable to [adverserial scenarios](https://github.com/waoai/confidis/blob/master/ATTACKS.md)
* Compare complex, multivariate answers

## Usage

### Javascript (NodeJS)

`npm install confidis` / `yarn add confidis`

In `nodejs`:

```javascript
const { GraphJS } = require("confidis/node")

GraphJS.execute_command("SET q1 a FROM s1")
GraphJS.execute_command("GET ANSWER TO q1") // { "cmd": "GetAnswer", confidience: 0.5, answer: "a" }
```

### Javascript (Browser)

`npm install confidis` / `yarn add confidis`

For browsers, make sure you're using webpack with a WebAssembly loader (for CRA users, you may need to eject). The loader goes under the "module"
key with the other loaders and looks like this:
```javascript
{
  test: /\.wasm$/,
  type: "webassembly/experimental",
}
```

When you import, make sure you use an async import.

```javascript
let confidis
import("confidis/webpack").then((c) => {
  confidis = c
})

// ...wait for confidis to load, it will be undefined until the import promise resolves

const GraphJS = confidis.GraphJS

GraphJS.execute_command("SET q1 a FROM s1")
GraphJS.execute_command("GET ANSWER TO q1") // { "cmd": "GetAnswer", confidience: 0.5, answer: "a" }
```

There is a working implementation in the `tryonline` folder that can be used for reference.

> We're really hoping `wasm-pack`, `webpack`, `create-react-app` and the rust-wasm-js ecosystem make this easier in the future. Many things were
> tried with limited success to get the solution above.

### Rust

Help wanted for this section.

## Terms

- question: An uncertain key.
- source: An entity, e.g. a person, who can supply answers
- answer: An answer to a question from a source. An uncertain value.
- comparison_method: A method that indicates the similarity of answers, and whether or not they are acceptably close.

## API

### Simple Query API

```bash
SET <question_id> <answer_content> FROM <source_id>

GET ANSWER TO <question_id>
# Returns { "confidence": 0.88, "answer": "someanswer" }


# Other commands
BELIEVE <source_id>
CLEAR ALL QUESTIONS
CLEAR ALL ANSWERS
GET SOURCES FOR <question_id>
GET BEST SOURCE FOR <question_id>
REMOVE ANSWER TO <question_id> FROM <source_id>
REMOVE QUESTION <question_id>

ADD ANSWER <answer_content> FOR <question_id> FROM <source_id>

# Configuring
CONFIGURE <configuration_setting> <value> [some_parameter=some_parameter_value ...]
```

### Configuration Settings

Each configuration parameter has a description in [graphs.rs](https://github.com/waoai/confidis/blob/master/src/graph.rs). Some
recommendations for adjusting configuration to mitigate attacks can be found in [ATTACKS.md](https://github.com/waoai/confidis/blob/master/ATTACKS.md).

| Setting Name                |  Default Value | Parameters                              |
| --------------------------- |  ------------- | --------------------------------------- |
| default_source_quality      |  0.5           |                                         |
| log_weight_factor           |  10.0          |                                         |
| initial_source_strength     |  1.0           |                                         |
| quality_of_believed_sources |  0.999         |                                         |
| comparison_method           |  exact         |                                         |
| comparison_method           |  numeric       | max_distance                            |
| comparison_method           |  numeric_vec   | vec_length, allowed_difference, diff_fn |
