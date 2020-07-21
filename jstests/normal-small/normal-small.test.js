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

const normvec = (v) => {
  const sum = v.reduce((acc, e) => acc + e, 0)
  return v.map((e) => e / sum)
}

const expspace = (first, last, factor, total) => {
  return range(total).map(
    (x) => first + (last - first) * factor ** (x / total - 1)
  )
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
      name: "Exponential Biased Selection",
      questions: normvec(expspace(0.01, 1, 100, 100)).map((answerBias) => ({
        answerBias,
      })),
    },
  ],
  totalUniqueAnswers: [2, 5, 100],
  totalAnswers: [250, 500, 1000],
  // knownQuestions: [0, 1, 10],
  sourcesConfig: [
    ...[5, 15 /* 50 */].flatMap((numSources) => [
      {
        name: "Lin Quantity, Lin Acc 0 - 100%",
        sources: createSources({
          accuracies: linspace(0, 1, numSources),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Lin Quantity, Lin Acc 0 - 50%",
        sources: createSources({
          accuracies: linspace(0, 0.5, numSources),
          answerBiases: linspace(0, 1, numSources),
        }),
      },
      {
        name: "Exp Quantity, Lin Acc 0 - 50%",
        sources: createSources({
          accuracies: linspace(0, 0.5, numSources),
          answerBiases: normvec(expspace(0.01, 1, 1000, numSources)),
        }),
      },
      // {
      //   name: "Lin Quantity, Lin Acc 75% - 100%",
      //   sources: createSources({
      //     accuracies: linspace(0.75, 1, numSources),
      //     answerBiases: linspace(0, 1, numSources),
      //   }),
      // },
      // {
      //   name: "Lin Quantity, Bad Quadratic Acc (x**2)",
      //   sources: createSources({
      //     accuracies: linspace(0, 1, numSources).map((x) => x ** 2),
      //     answerBiases: linspace(0, 1, numSources),
      //   }),
      // },
      // {
      //   name: "Lin Quantity, Good Quadratic Acc (1-x**2)",
      //   sources: createSources({
      //     accuracies: linspace(0, 1, numSources).map((x) => 1 - x ** 2),
      //     answerBiases: linspace(0, 1, numSources),
      //   }),
      // },
      // {
      //   name: "Quadratic Quantity, Bad Quadratic Acc (x**2)",
      //   sources: createSources({
      //     accuracies: linspace(0, 1, numSources).map((x) => x ** 2),
      //     answerBiases: linspace(0, 1, numSources).map((x) => x ** 2),
      //   }),
      // },
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

  // console.log(sources)

  const trials = 10
  test(testName, (t) => {
    t.timeout(1000 * 30, "Each test must complete in less than 30 seconds")
    for (let trial = 0; trial < trials; trial++) {
      const rng = seedrandom(testName + trial)
      const g = GraphJS.new()
      g.execute_command("CONFIGURE initial_source_strength 10.0")
      g.execute_command("CONFIGURE default_source_quality 0.5")
      g.execute_command("CONFIGURE log_weight_factor 100.0")

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
      let sourceAnswers = range(sources.length).reduce(
        (acc, i) => ({ ...acc, [`s${i}`]: {} }),
        {}
      )
      while (numberAnswered < totalAnswers) {
        const selectedQuestion = weighted.select(questionChanceOfSelection, {
          rand: rng,
        })
        const selectedSource = weighted.select(sourceChanceOfSelection, {
          rand: rng,
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
        sourceAnswers[selectedSource][selectedQuestion] = answer
        const cmdString = `SET ${selectedQuestion} ${answer} FROM ${selectedSource}`
        // console.log(cmdString)
        g.execute_command(cmdString)
        numberAnswered++
      }

      // Force convergence
      // for (let k = 0; k < 10; k++) {
      //   const questionIndices = range(questions.length)
      //   shuffle(questionIndices)
      //   for (const i of questionIndices) {
      //     const answers = Object.values(questionAnswerFromSource[`q${i}`])
      //     if (answers.length === 0) continue
      //     g.execute_command(`GET ANSWER TO q${i}`)
      //   }
      // }

      const gAnswers = {}
      const mvAnswers = {}
      for (const i of range(questions.length)) {
        const answers = Object.values(questionAnswerFromSource[`q${i}`])
        if (answers.length === 0) continue
        const freqList = mostCommon(answers.map((a) => a.toString()))
        // Select the highest frequency, then randomize the one selected
        const highestFreq = freqList.filter((l) => l.count >= freqList[0].count)
        shuffle(highestFreq, { rng })

        mvAnswers[`q${i}`] = parseInt(highestFreq[0].token)
        const { answer, confidence } = g.execute_command(`GET ANSWER TO q${i}`)
        gAnswers[`q${i}`] = parseInt(answer)
      }

      const numberPossibleToGetCorrect = Object.values(
        questionAnswerFromSource
      ).filter((answerMap) => Object.values(answerMap).includes(0)).length

      const gAcc =
        Object.values(gAnswers).filter((a) => a === 0).length /
        numberPossibleToGetCorrect
      const mvAcc =
        Object.values(mvAnswers).filter((a) => a === 0).length /
        numberPossibleToGetCorrect

      testResults.push({ testName, gAcc, mvAcc })
      // t.assert(gAcc >= mvAcc - 0.2)
      g.free()
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

test("overall score", (t) => {
  const gAccAvg =
    testResults.reduce((acc, { gAcc }) => acc + gAcc, 0) / testResults.length
  const mvAccAvg =
    testResults.reduce((acc, { mvAcc }) => acc + mvAcc, 0) / testResults.length
  console.log("=================================")
  console.log(
    "Overall Score: ",
    (gAccAvg - mvAccAvg).toFixed(3),
    "(",
    gAccAvg.toFixed(2),
    mvAccAvg.toFixed(2),
    ")"
  )
  t.pass()
})
