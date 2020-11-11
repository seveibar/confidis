const test = require("ava")
const { GraphJS } = require("../pkg/node")

test("custom javascript equalifier", async (t) => {
    const g = GraphJS.new_with_equalifier((ann1, ann2) => {
        return Math.min(1, Math.abs(parseFloat(ann1) - parseFloat(ann2)))
    })

    t.is(g.execute_command("TEST EQUALITY 1 1.5").distance, 0.5);
})