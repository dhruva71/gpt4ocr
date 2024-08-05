# GPT4OCR
A simple OCR tool that uses GPT-4o to perform OCR on pdf files.
Requires a .env file with the following variables:
```
OPENAI_API_KEY=your_openai_api_key
```

# Operating systems
Runs on linux. Needs poppler-utils to be installed. To install it on Ubuntu, run
`sudo apt install poppler-utils`
`sudo apt install libssl-dev`

# Important observations
* The time grows with the number of fields generated. You can specify the JSON format to limit the number of fields generated in the prompt, and that can help reduce the time required.
* JSON comes back as a markdown block, so you can remove the "```json" and "```" to get the JSON data (handled by the library).