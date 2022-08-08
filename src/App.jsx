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

  return (
    <Container className='App'>
      <Heading>Fastq Analyser</Heading>
      {/* The input options */}
      <Accordion>
        <AccordionItem>
          <AccordionButton>
            <Text>Input Text</Text>
            <Spacer />
            <AccordionIcon />
          </AccordionButton>
          <AccordionPanel>
            <TextInput title={'Paste fastq'} onChange={handleTextInput}/>
          </AccordionPanel>
        </AccordionItem>

        <AccordionItem>
          <AccordionButton>
            <Text>Input File</Text>
            <Spacer />
            <AccordionIcon />
          </AccordionButton>
          <AccordionPanel>
            <FileInput title={'Upload Fastq file'} onChange={handleFileInput} />
          </AccordionPanel>
        </AccordionItem>
      </Accordion>

      {/* Control buttons */}
      <Center>
        <ButtonGroup spacing={4}>
          <Button colorScheme='blue'>Submit</Button>
          <Button variant='outline' colorScheme='red'>Clear</Button>
        </ButtonGroup>
      </Center>
    </Container>
  )
}

export default App
