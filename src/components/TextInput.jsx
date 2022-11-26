import {
  FormControl,
  FormLabel,
  FormErrorMessage,
  Textarea,
} from '@chakra-ui/react'

function TextInput({isInvalid = false, title, onChange, id}) {
  return (
    <FormControl isInvalid={isInvalid}>
      <FormLabel>{title}</FormLabel>
      <Textarea
        id={id}
        placeholder='Paste one or more sequence records.'
        onChange={onChange}
      />
      {isInvalid && 
          <FormErrorMessage>
            Please paste at least one valid sequence record.
          </FormErrorMessage>
      }
    </FormControl>
  )
}

export default TextInput