import {
  FormControl,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Input
} from '@chakra-ui/react';

function FileInput({ isInvalid = false, title, onChange, id }) {
  return (
    <FormControl isInvalid={isInvalid}>
      <FormLabel>{title}</FormLabel>
      <Input type='file' onChange={onChange} id={id} />
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
  );
}

export default FileInput;
