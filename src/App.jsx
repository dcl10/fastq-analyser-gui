import { useState } from 'react'
import { 
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionPanel,
  AccordionIcon,
  Button,
  ButtonGroup,
  Box,
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
            <TextInput title={'Paste fastq'} />
          </AccordionPanel>
        </AccordionItem>

        <AccordionItem>
          <AccordionButton>
            <Text>Input File</Text>
            <Spacer />
            <AccordionIcon />
          </AccordionButton>
          <AccordionPanel>
            <FileInput title={'Upload Fastq file'}/>
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
