import {
  FormControl,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Textarea,
} from '@chakra-ui/react'

function TextInput({isInvalid = false, title, onChange}) {
  return (
    <FormControl isInvalid={isInvalid}>
      <FormLabel>{title}</FormLabel>
      <Textarea 
        placeholder='Paste one or more fastq records.'
        onChange={onChange}
      />
      {isInvalid && 
          <FormErrorMessage>
            Please paste at least one valid fastq record.
          </FormErrorMessage>
      }
    </FormControl>
  )
}

export default TextInput