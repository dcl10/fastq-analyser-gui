// Send the sequence text to the backend and return the analytics
const analyseTextSequences = async (sequences, format) => {
    let choice;
    switch (format) {
        case "fastq":
            choice = "analyse_fastq_sequences"
            break
        case "fasta":
            choice = "analyse_fasta_sequences"
            break
        default: throw new Error(`Choice "${format}" is not a valid command`)
    }
    let results = await invoke(choice, {sequences})
    return results
}

// Send the sequene file to the backend and return the analytics
const analyseFileSequences = async (path, format) => {
    let choice;
    switch (format) {
        case "fastq":
            choice = "analyse_fastq_file"
            break
        case "fasta":
            choice = "analyse_fasta_file"
            break
        default: throw new Error(`Choice "${format}" is not a valid command`)
    }
    let results = await invoke(choice, {path})
    return results
}

export { analyseFileSequences, analyseTextSequences }