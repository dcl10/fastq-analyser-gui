import {
  FormControl,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Input
} from '@chakra-ui/react'

function TextInput({isInvalid = false, title}) {
  return (
    <FormControl isInvalid={isInvalid}>
      <FormLabel>{title}</FormLabel>
      <Input />
      {
        isInvalid ? (
          <FormErrorMessage>
            Please paste at least one valid fastq record.
          </FormErrorMessage>
        ) : (
          <FormHelperText>
            Paste one or more fastq records.
          </FormHelperText>
        )
      }
    </FormControl>
  )
}

export default TextInput