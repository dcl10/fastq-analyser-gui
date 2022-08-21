import {
  Button,
  FormControl,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Input,
  InputGroup,
  InputLeftAddon,
} from '@chakra-ui/react'

function FileInput({isInvalid = false, title, onClick, id}) {
  return (
    <FormControl isInvalid={isInvalid}>
      <FormLabel>{title}</FormLabel>
      <InputGroup>
        <InputLeftAddon>
          <Button size={'sm'}  onClick={onClick}>Select</Button>
        </InputLeftAddon>
        <Input isReadOnly id={id} />
      </InputGroup>
      {
        isInvalid ? (
          <FormErrorMessage>
            Please select a valid file.
          </FormErrorMessage>
        ) : (
          <FormHelperText>
            Upload a fastq file with DNA sequences.
          </FormHelperText>
        )
      }
    </FormControl>
  )
}

export default FileInput