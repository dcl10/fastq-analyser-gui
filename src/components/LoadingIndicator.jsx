import { HStack, Spinner, Text } from "@chakra-ui/react";

function LoadingIndicator ({ message }) {
  return (
    <HStack>
      <Spinner
        thickness='4px'
        speed='0.65s'
        emptyColor='gray.200'
        color='blue.500'
        size='xl'
      />
      <Text>{message}</Text>
    </HStack>
  )
}

export default LoadingIndicator