import React from "react"
import { Box } from "@material-ui/core"

export default ({ children }) => (
  <Box display="flex" justifyContent="center">
    <Box maxWidth={1200} width="100%">
      {children}
    </Box>
  </Box>
)
