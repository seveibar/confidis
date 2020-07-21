import React, { useState, useEffect } from "react"
import { Box, Grid, styled, colors } from "@material-ui/core"
import ExpandingTextArea from "react-expanding-textarea"

let confidis
import("confidis/webpack").then((c) => {
  confidis = c
})

const defaultProgram = `

SET q1 a FROM s1
SET q1 a FROM s2
SET q1 a FROM s3
SET q1 w FROM s4

SET q2 b FROM s1
SET q2 c FROM s2
SET q2 b FROM s3
SET q2 w FROM s4

SET q3 d FROM s1
SET q4 w FROM s4

GET ANSWER TO q1
GET ANSWER TO q2
GET ANSWER TO q3
GET ANSWER TO q4

`.trim()

const Container = styled(Box)({ padding: 16 })
const TextArea = styled(ExpandingTextArea)({
  fontFamily: "monospace",
  boxSizing: "border-box",
  width: "100%",
  fontSize: 24,
  minHeight: 600,
  padding: 16,
  border: `1px solid ${colors.grey[500]}`,
  resize: "both",
  overflow: "auto",
})
const TryItOut = styled(Box)({
  display: "inline-block",
  fontWeight: 900,
  fontSize: 24,
  color: colors.grey[800],
  borderBottom: `2px solid ${colors.grey[400]}`,
  paddingBottom: 6,
  paddingLeft: 4,
  paddingRight: 16,
  marginBottom: 16,
})

export default () => {
  const [text, setText] = useState(defaultProgram)
  const [output, setOutput] = useState("Computing in WebAssembly...")
  const [confidisLoaded, setConfidisLoaded] = useState(false)

  useEffect(() => {
    if (confidisLoaded) return
    const interval = setInterval(() => {
      if (confidis) {
        setConfidisLoaded(true)
        clearInterval(interval)
      }
    }, 100)
    return () => {
      clearInterval(interval)
    }
  }, [confidisLoaded, setConfidisLoaded])

  useEffect(() => {
    if (!confidis) {
      return
    }
    const g = confidis.GraphJS.new()
    const output = []
    for (const line of text.split("\n")) {
      try {
        const result = g.execute_command(line.trim())
        if (result.cmd === "GetAnswer") {
          output.push(
            `${result.answer} (${(result.confidence * 100).toFixed(3)}%)`
          )
        } else if (result.cmd === "GetSource") {
          output.push(`${result.quality.toFixed(3)}`)
        } else {
          output.push("")
        }
      } catch (e) {
        if (e.toString() === "Blank command") {
          output.push("")
        } else {
          output.push("Err: " + e.toString())
        }
      }
    }
    setOutput(output.join("\n"))
  }, [text, confidisLoaded])

  return (
    <Container>
      <Grid container>
        <Grid item xs={12}>
          <Box display="flex">
            <TryItOut>Try It Out</TryItOut>
            <Box flexGrow={1} />
          </Box>
          <Box style={{ position: "relative" }}>
            <TextArea
              contentEditable
              value={text}
              onChange={(e) => setText(e.target.value)}
            />
            <TextArea
              value={output}
              style={{
                display: "inline-block",
                pointerEvents: "none",
                position: "absolute",
                top: 0,
                right: 0,
                border: "none",
                backgroundColor: "transparent",
                width: 400,
                opacity: 0.5,
              }}
            />
          </Box>
        </Grid>
        {/* <Grid item xs={4}> */}
        {/* PUT GRAPH HERE */}
        {/* </Grid> */}
      </Grid>
    </Container>
  )
}
