import {
  FormControl,
  FormLabel,
  Switch,
} from '@chakra-ui/react'

function ToggleSwitch({title, onChange, id, value, isChecked}) {
    return (
        <FormControl display='flex' alignItems='center'>
            <FormLabel>{title}: {value}</FormLabel>
            <Switch id={id} onChange={onChange} isChecked={isChecked} value={value} />
        </FormControl>
    )
}

export default ToggleSwitch