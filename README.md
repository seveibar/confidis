# Confidis

Confidis is a key store for uncertain values from multiple disagreeing sources.

## Terms

* question
* source
* answer

## API

### Simple Query API

```bash
ADD QUESTION <question_id> <question_content> <question_type>

GET <question_id>
# Returns { "confidence": 0.88, "answer": 123, "correctSources": Array<source_id> }

ADD ANSWER <answer_content> FOR <question_id> FROM <source_id>
```


# Old Stuff

## API

| Endpoint           | Params                       |
| ------------------ | ---------------------------- |
| GET `/assign`      | `worker_id`, `task_group_id` |
| GET `/confidences` | `task_group_id`              |

### Debug Endpoints

| Endpoint                 | Params                                        |
| ------------------------ | --------------------------------------------- |
| GET `/debug/matrix`      | `task_group_id`                               |
| GET `/debug/matrix/item` | `task_group_id`, `task_index`, `worker_index` |

## Internals

The microservice must process data in a series of stages.

### 1. Database Caller

This stage converts the data into the standard game format. Each `{...}` is the content of a `task_fulfillment`.

| Method     | Parameters                   | Output                       |
| ---------- | ---------------------------- | ---------------------------- |
| `loadGame` | `task_group_id`, `worker_id` | `{ game: ContentGameBoard }` |

#### ContentGameBoard

|         | W_p     | w_1     | w_2     | w_3     | w_4     |
| ------- | ------- | ------- | ------- | ------- | ------- |
| **t_1** | `{...}` | `{...}` | `{...}` | `{...}` | `{...}` |
| **t_2** | `{...}` | `{...}` | `{...}` | `{...}` | `{...}` |
| **t_3** | `{...}` | `{...}` | null    | `{...}` | `{...}` |
| **t_4** | `{...}` | `{...}` | null    | `{...}` | `{...}` |
| **t_5** | `{...}` | `{...}` | null    | `{...}` | null    |
| **t_6** | `{...}` | `{...}` | null    | `{...}` | null    |
| **t_7** | `{...}` | null    | null    | `{...}` | null    |

### 2. Equalifer

This stage converts the content for each task

This stage converts each task row into a similarity matrix that shows how related each answer is to each other answer. The output is a 3d matrix.

_note: The equalifier should not receive any `null` rows_

|         | W_p | w_1 | w_2 | w_3 | w_4 |
| ------- | --- | --- | --- | --- | --- |
| **t_1** | 1.0 | 0.1 | 0.2 | 0.0 | 0.0 |
|         | 0.1 | 1.0 | 0.0 | 0.0 | 0.0 |
|         | 0.2 | 0.0 | 1.0 | 0.5 | 0.0 |
|         | 0.0 | 0.0 | 0.5 | 1.0 | 0.0 |
|         | 0.0 | 0.0 | 0.0 | 0.0 | 1.0 |
| **t_2** | ... | ... | ... | ... | ... |

### 3. Cluster and Select

This stage converts the equalifer output into a grouping of similar answers such that they can be compared to eachother.

|         | W_p | w_1 | w_2 | w_3 | w_4 |
| ------- | --- | --- | --- | --- | --- |
| **t_1** | 1   | 1   | 2   | 2   | 3   |
| **t_2** | ... | ... | ... | ... | ... |
| **t_3** | ... | ... | ... | ... | ... |
| **t_4** | ... | ... | ... | ... | ... |
| **t_5** | ... | ... | ... | ... | ... |

### 4. Compute Confidences and Worker Accuracies

Use the clusterer output to find the confidence in each task as well as the accuracy of each worker (2 outputs).

#### Task Confidence Table

|         | Answer | Confidence |
| ------- | ------ | ---------- |
| **t_1** | 1      | 0.98       |
| **t_2** | 2      | 0.88       |
| **t_3** | 3      | 0.79       |
| **t_4** | 4      | 0.40       |
| **t_5** | 0      | 0.00       |

#### Worker Accuracies

|         | Accuracy | Number Completed |
| ------- | -------- | ---------------- |
| **w_1** | 0.93     | 88               |
| **w_2** | 0.80     | 10               |
| **w_3** | 0.82     | 16               |
| **w_4** | 0.76     | 8                |
| **w_5** | 0.40     | 4                |

### 5. Compute Worker Assignment

Compute the assignment for the a given worker index.
