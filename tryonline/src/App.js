import React from "react"
import { Box, styled, Grid, colors } from "@material-ui/core"
import InnerContent from "./InnerContent"
import Playground from "./Playground"

const Container = styled(Box)({ marginTop: 40 })
const Top = styled(Box)({
  // borderBottom: `1px solid ${colors.grey[400]}`,
  paddingBottom: 40,
})
const TitleSection = styled(Box)({ padding: 20 })
const LinksSection = styled(Box)({
  paddingTop: 80,
  color: colors.grey[600],
  "& a": {
    color: colors.blue[500],
  },
})
const Title = styled(Box)({
  fontWeight: 900,
  fontSize: 96,
})
const SubTitle = styled(Box)({
  fontWeight: 800,
  fontSize: 48,
  marginTop: 20,
})
const About = styled(Box)({
  fontSize: 18,
  lineHeight: 1.5,
  padding: 8,
})
const Bottom = styled(Box)({})

function App() {
  return (
    <Container>
      <InnerContent>
        <Top>
          <Grid container>
            <Grid xs={12} md={8} item>
              <TitleSection>
                <Title>Confidis</Title>
                <SubTitle>
                  A probabilistic key store for finding truth from arguing
                  sources
                </SubTitle>
              </TitleSection>
            </Grid>
            <Grid xs={12} md={4} item>
              <LinksSection>
                <About>
                  Confidis focuses on ease-of-use, correctness, performance, and
                  resilience against adverserial scenarios. The library is
                  written in Rust and can be used as an npm module.
                  <br />
                  <br />
                  Confidis was originally designed by{" "}
                  <a href="https://twitter.com/seveibar">@seveibar</a> to
                  analyze disagreeing data sources for data aggregation. It was
                  further developed and sponsored by{" "}
                  <a href="https://wao.ai">wao.ai</a>.
                  <br />
                  <br />
                  <a href="https://github.com/waoai/confidis">
                    Read more on the README.md
                  </a>
                </About>
              </LinksSection>
            </Grid>
          </Grid>
        </Top>
        <Bottom>
          <Playground />
        </Bottom>
      </InnerContent>
    </Container>
  )
}

export default App
