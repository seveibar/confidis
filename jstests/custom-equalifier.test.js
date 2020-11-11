const test = require("ava")
const { GraphJS } = require("../../pkg/node")

test("custom javascript equalifier", async (t) => {
    const g = GraphJS.new_with_equalifier((ann1, ann2) => {
        return Math.max(1, Math.abs(parseFloat(ann1) - parseFloat(ann2)))
    })


})