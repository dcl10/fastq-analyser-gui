import { Text } from '@chakra-ui/react'

function FastqResultPanel({ result }) {
    return (
        <>
            <Text>
                <strong>Description:</strong>&nbsp;{result.desc}
            </Text>
            <Text>
                <strong>Record is valid?</strong>&nbsp;{result.is_valid ? 'Yes' : 'No'}
            </Text>
            <Text>
                <strong>Sequence length:</strong>&nbsp;{result.seq_len} bases
            </Text>
            <Text>
                <strong>PHRED score per base:</strong>&nbsp;{result.phred_score / result.seq_len}
            </Text>
            <Text>
                <strong>GC %:</strong>&nbsp;{result.gc * 100}%
            </Text>
            <Text>
                <strong>No.# ORFs:</strong>&nbsp;{result.n_orfs}
            </Text>
        </>
    )
}

export default FastqResultPanel