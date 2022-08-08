import {
  FormControl,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Textarea,
} from '@chakra-ui/react'

function TextInput({isInvalid = false, title}) {
  return (
    <FormControl isInvalid={isInvalid}>
      <FormLabel>{title}</FormLabel>
      <Textarea placeholder='Paste one or more fastq records.' />
      {isInvalid && 
          <FormErrorMessage>
            Please paste at least one valid fastq record.
          </FormErrorMessage>
      }
    </FormControl>
  )
}

export default TextInput