import React, { useState } from "react"
import { Box, Grid, styled, colors } from "@material-ui/core"
import ExpandingTextArea from "react-expanding-textarea"

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
  const [output, setOutput] = useState("Some output \n\n\nsome more output")
  return (
    <Container>
      <Grid container>
        <Grid item xs={8}>
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
                width: 300,
                opacity: 0.5,
              }}
            />
          </Box>
        </Grid>
        <Grid item xs={4}>
          {/*  */}
        </Grid>
      </Grid>
    </Container>
  )
}
