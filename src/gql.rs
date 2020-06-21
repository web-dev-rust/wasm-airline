pub fn fetch_gql() -> String {
    format!("
{{
    \"query\": \"
    {{
        bestPrices(departure: \\\"2020-07-21\\\", 
          origin: \\\"POA\\\", 
          destination: \\\"GRU\\\") {{
          bestPrices {{
            date
            available
            price {{amount}}
          }}
        }}
      }}
    \"
}}
    ")
}