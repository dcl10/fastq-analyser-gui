import { useState } from 'react'
import { 
  Button,
  Container,
  Heading,
  HStack,
  Input,
  Text,
} from '@chakra-ui/react'
import { invoke } from '@tauri-apps/api'

function App() {
  const [count, setCount] = useState(0)

  /**
   * A slightly contrived way to increase the count
   * to show how to invoke Rust commands from the frontend
   */
  const rustIncrease = async () => {
    let newNumber = await invoke('increase', {num: count})
    setCount(newNumber)
  }

  /**
   * A slightly contrived way to decrease the count
   * to show how to invoke Rust commands from the frontend
   */
  const rustDecrease = async () => {
    let newNumber = await invoke('decrease', {num: count})
    setCount(newNumber)
  }

  return (
    <Container className='App'>
      <Heading>This is a template Tauri app.</Heading>
      <Text>It combines a fast and safe Rust backend with a React frontend.</Text>
      <HStack>
        <Button onClick={rustDecrease}>-</Button>
        <Input value={count} isReadOnly />
        <Button onClick={rustIncrease}>+</Button>
      </HStack>
    </Container>
  )
}

export default App
