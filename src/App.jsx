import { useState } from 'react'
import { 
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionPanel,
  AccordionIcon,
  Button,
  ButtonGroup,
  Container,
  Heading,
  Text,
  Spacer,
  Center,
} from '@chakra-ui/react'
import { invoke } from '@tauri-apps/api'
import FileInput from './components/FileInput'
import TextInput from './components/TextInput'

function App() {
  const [textSequences, setTextSequences] = useState('')
  const [fileSequences, setFileSequences] = useState()
  const [results, setResults] = useState({})

  // Change the text sequences in state 
  const handleTextInput = (event) => {
    let newValue = event.target.value
    setTextSequences(newValue)
  }

  // Change the file sequences in state
  const handleFileInput = (event) => {
    let newValue = event.target.files[0]
    setFileSequences(newValue)
  }

  // Clear the input fields and reset the state
  const clearInputs = () => {
    let textInput = document.getElementById('text-input')
    textInput.value = ''
    setTextSequences('')

    let fileInput = document.getElementById('file-input')
    fileInput.value = ''
    setFileSequences(undefined)
  }

  // Send the text sequences to the backend and return the analytics
  const analyseSequences = async () => {
    let results = await invoke('analyse_sequences', {sequences: textSequences})
    setResults(results)
  }

  return (
    <Container className='App'>
      <Heading>Fastq Analyser</Heading>
      {/* The input options */}
      <Accordion allowMultiple allowToggle>
        <AccordionItem>
          <AccordionButton>
            <Text>Input Text</Text>
            <Spacer />
            <AccordionIcon />
          </AccordionButton>
          <AccordionPanel>
            <TextInput id='text-input' title={'Paste fastq'} onChange={handleTextInput}/>
          </AccordionPanel>
        </AccordionItem>

        <AccordionItem isDisabled>
          <AccordionButton>
            <Text>Input File</Text>
            <Spacer />
            <AccordionIcon />
          </AccordionButton>
          <AccordionPanel>
            <FileInput id='file-input' title={'Upload Fastq file'} onChange={handleFileInput} />
          </AccordionPanel>
        </AccordionItem>
      </Accordion>

      {/* Control buttons */}
      <Center>
        <ButtonGroup spacing={4}>
          <Button
            colorScheme='blue'
            onClick={analyseSequences}
          >
            Submit
          </Button>
          <Button
            variant='outline'
            colorScheme='red'
            onClick={clearInputs}
          >
            Clear
          </Button>
        </ButtonGroup>
      </Center>
    </Container>
  )
}

export default App
