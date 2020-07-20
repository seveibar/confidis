const linspace = require("linspace")
const combos = require("combos")
const test = require("ava")
const range = require("lodash/range")
const { GraphJS } = require("../../pkg/node")
const seedrandom = require("seedrandom")
const weighted = require("weighted")
const shuffle = require("shuffle-array")
const mostCommon = require("most-common")

const globalRng = seedrandom("globalRng")
const createSources = ({ accuracies, answerBiases }) => {
  const ilist = range(accuracies.length)
  shuffle(ilist, { rng: globalRng })
  return accuracies.map((accuracy, i) => ({
    accuracy,
    answerBias: answerBiases[ilist[i]],
  }))
}

const tests = combos({
  questionConfig: [
    {
      name: "Uniform Selection",
      questions: range(100).map((i) => ({ answerBias: 1 })),
    },
    {
      name: "Linearly Biased Selection",
      questions: range(100).map((i) => ({ answerBias: i })),
    },
    {
      name: "10x Quadratic Biased Selection",
      questions: range(100).map((i) => ({
        answerBias: 1 + 9 * (i / 100) ** 2,
      })),
    },
  ],
  totalUniqueAnswers: [2, 5, 100],
  totalAnswers: [250, 500, 1000],
  sourcesConfig: [
    ...[10, 20, 50].flatMap((numSources) => [
      {
        name: "Lin Quantity, Lin Acc 0 - 100%",
        sources: createSources({
          accuracies: linspace(0, 1, numSources),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Lin Quantity, Lin Acc 0 - 25%",
        sources: createSources({
          accuracies: linspace(0, 0.25, numSources),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Lin Quantity, Lin Acc 75% - 100%",
        sources: createSources({
          accuracies: linspace(0.75, 1, numSources),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Lin Quantity, Bad Quadratic Acc (x**2)",
        sources: createSources({
          accuracies: linspace(0, 1, numSources).map((x) => x ** 2),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Lin Quantity, Good Quadratic Acc (1-x**2)",
        sources: createSources({
          accuracies: linspace(0, 1, numSources).map((x) => 1 - x ** 2),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Quadratic Quantity, Bad Quadratic Acc (1-x**2)",
        sources: createSources({
          accuracies: linspace(0, 1, numSources).map((x) => x ** 2),
          answerBiases: linspace(0, 1, numSources).map((x) => x ** 2),
        }),
      },
    ]),
  ],
})

const testResults = []
for (const testConfig of tests) {
  const {
    totalAnswers,
    totalUniqueAnswers,
    questionConfig: { questions, name: questionConfigName },
    sourcesConfig: { name: sourceConfigName, sources },
  } = testConfig
  const testName = `${
    questions.length
  } Questions w/ ${questionConfigName}, ${totalAnswers} Answers, guess chance: ${Math.round(
    (1 / totalUniqueAnswers) * 100
  )}%, ${sources.length} Srcs: ${sourceConfigName}`

  const trials = 5
  test(testName, (t) => {
    for (let trial = 0; trial < trials; trial++) {
      const rng = seedrandom(testName + trial)
      const g = GraphJS.new()
      g.execute_command("CONFIGURE initial_source_strength 50.0")
      g.execute_command("CONFIGURE default_source_quality 0.01")

      let numberAnswered = 0
      const questionChanceOfSelection = questions.reduce(
        (acc, q, i) => ({ ...acc, [`q${i}`]: q.answerBias }),
        {}
      )
      const sourceChanceOfSelection = sources.reduce(
        (acc, source, i) => ({ ...acc, [`s${i}`]: source.answerBias || 1 }),
        {}
      )
      let questionAnswerFromSource = range(questions.length).reduce(
        (acc, i) => ({ ...acc, [`q${i}`]: {} }),
        {}
      )
      while (numberAnswered < totalAnswers) {
        const selectedQuestion = weighted.select(questionChanceOfSelection, {
          rand: rng,
          normalize: true,
        })
        const selectedSource = weighted.select(sourceChanceOfSelection, {
          rand: rng,
          normalize: true,
        })
        // no duplicate answers
        if (
          questionAnswerFromSource[selectedQuestion][selectedSource] !==
          undefined
        ) {
          numberAnswered += 1 / 20
          continue
        }

        const isSourceCorrect =
          rng() < sources[parseInt(selectedSource.slice(1))].accuracy
        const answer = isSourceCorrect
          ? 0
          : Math.floor(rng() * totalUniqueAnswers)
        questionAnswerFromSource[selectedQuestion][selectedSource] = answer
        g.execute_command(
          `SET ${selectedQuestion} ${answer} FROM ${selectedSource}`
        )
        numberAnswered++
      }

      const gAnswers = {}
      const mvAnswers = {}
      for (const i of range(questions.length)) {
        const answers = Object.values(questionAnswerFromSource[`q${i}`])
        if (answers.length === 0) continue
        mvAnswers[`q${i}`] = parseInt(
          mostCommon(answers.map((a) => a.toString()))[0].token
        )
        const { answer, confidence } = g.execute_command(`GET ANSWER TO q${i}`)
        gAnswers[`q${i}`] = parseInt(answer)
      }

      const gAcc =
        Object.values(gAnswers).filter((a) => a === 0).length /
        Object.values(gAnswers).length
      const mvAcc =
        Object.values(mvAnswers).filter((a) => a === 0).length /
        Object.values(gAnswers).length

      testResults.push({ testName, gAcc, mvAcc })
      // t.assert(gAcc >= mvAcc - 0.2)
    }
    const relevantResults = testResults.filter(
      ({ testName: tn }) => tn === testName
    )
    const gAccAvg =
      relevantResults.reduce((acc, { gAcc }) => acc + gAcc, 0) /
      relevantResults.length
    const mvAccAvg =
      relevantResults.reduce((acc, { mvAcc }) => acc + mvAcc, 0) /
      relevantResults.length
    console.log(testName.padEnd(150), gAccAvg.toFixed(2), mvAccAvg.toFixed(2))
    t.pass(testName)
  })
}
