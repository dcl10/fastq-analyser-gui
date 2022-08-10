import { useRef, useState } from 'react'
import { 
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionPanel,
  AccordionIcon,
  Button,
  ButtonGroup,
  Center,
  Container,
  Heading,
  Text,
  useDisclosure,
  Spacer,
} from '@chakra-ui/react'
import { invoke } from '@tauri-apps/api'
import FileInput from './components/FileInput'
import FQModal from './components/FQModal'
import LoadingIndicator from './components/LoadingIndicator'
import TextInput from './components/TextInput'

function App() {
  const textSequences = useRef('')
  const fileSequences = useRef()
  const [results, setResults] = useState([])

  const { isOpen, onOpen, onClose } = useDisclosure()

  // Change the text sequences in state 
  const handleTextInput = (event) => {
    textSequences.current = event.target.value
  }

  // Change the file sequences in state
  const handleFileInput = (event) => {
    fileSequences.current = event.target.files[0]
  }

  // Clear the input fields and reset the state
  const clearInputs = () => {
    let textInput = document.getElementById('text-input')
    textInput.value = ''
    textSequences.current = ''

    let fileInput = document.getElementById('file-input')
    fileInput.value = ''
    fileSequences.current = undefined
  }

  // Send the text sequences to the backend and return the analytics
  const analyseSequences = async () => {
    onOpen()
    let results = await invoke('analyse_sequences', {sequences: textSequences.current})
    setResults(results)
  }

  return (
    <Container className='App'>
      {/* Results modal */}
      <FQModal 
        title={'Results'}
        isOpen={isOpen}
        onClose={onClose}
      >
        {results.length > 0 ? (
            <Accordion allowMultiple allowToggle>
              {
                results.map(
                  result => (
                    <AccordionItem>
                      <AccordionButton>
                        <Heading as='h4' size='md'>
                          {result.id}
                        </Heading>
                        <Spacer />
                        <AccordionIcon />
                      </AccordionButton>
                      <AccordionPanel>
                        <Text>
                          <strong>Description:</strong>&nbsp;{result.desc}
                        </Text>
                        <Text>
                          <strong>Record is valid?</strong>&nbsp;{result.is_valid ? 'Yes': 'No'}
                        </Text>
                        <Text>
                          <strong>GC %:</strong>&nbsp;{result.gc * 100}%
                        </Text>
                        <Text>
                          <strong>No.# ORFs:</strong>&nbsp;{result.n_orfs}
                        </Text>
                      </AccordionPanel>
                    </AccordionItem>
                  )
                )
              }
            </Accordion>
          ) : (
            <Center>
              <LoadingIndicator message={'Loading results...'}/>
            </Center>
          )
        }
      </FQModal>

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
