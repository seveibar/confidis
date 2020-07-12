# Attacks

Truth discovery systems must be resilient to different "attacks" wherein a source tries to corrupt
the truth and confidences of the system. By understanding each type of attack and the susceptibility
of our configuration or environment to different scenarios, we can tune the system to eliminate the
attacks.


## Start Good, Turn Bad

In this attack, a source will try to be as accurate as possible for their first N entries, then they
will lie, corrupting future entries with their incorrect answers. The system initially gave them a
high quality score, so they're 

To increase the effort to perform this attack, increase the `initial_source_strength`, which will bound
the source to a lower accuracy for a longer number of entries. If you are in control of source querying,
periodic blind validation mitigates this attack well.

## Popular, incorrect answer

In this attack, many sources will believe an incorrect answer. Because so many sources believe it (often
the probability of guessing it is high) the incorrect answer obtains a higher-than-normal confidence.


A future mitigation, ["confidence limited to maximum source qualities"](https://github.com/waoai/confidis/issues/2)
addresses this issue well, because poor sources can never override accurate sources.

## Angry Mob / Biased Groups

Biased groups or angry mobs introduce a consistent bias in their answers, which creates two or more answer
clusters of a high liklihood. This isn't a problem if one bias is considered correct, because you would
be able to `BELIEVE` sources from the target bias. However, if sources consistently group into a small
number of biases, 

This can be solved by turning on multi-truth mode (if multiple truths are acceptable). This is not implemented yet.

A future mitigation may introduce `DONT BELIEVE` to mark sources as being low quality.

A future mitigation may introduce single competing truth mode, in which the confidence of each "incorrect" answer
causes the accurate answer's confidence to decrease. e.g.

`new_confidence = (old_confidence * -log_10(old_confidence) - SUM(other_confidence_i * -log_10(other_confidence_i)) / SUM(all_confidence_i * -log_10(all_confidence_i)`

The example above weights the confidence of the selected answer (old_confidence) against other confidences as a function of the strength of the confidence.

## Know-it-all source can't be proven wrong

A source can't be proven wrong if it's quality and strength greatly exceeds that of all other sources. If the source
is constantly answering many questions, it will reduce the quality of other sources, thus securing it's strength.

Reducing `strength_maximum`, e.g. `CONFIGURE strength_maximum <LOW ~10-100>` of the graph will allow other sources
to gain quality faster and override the know-it-all.

This is mostly mitigated by having a validation source that is `BELIEVE`'d.

Can be mitigated by restricting what percentage of questions a know-it-all is considered for.
