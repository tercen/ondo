database_server *-> domain 
    *-> table *-> table_value
    *-> table *-> index *-> index_value

//TODO: The same id generator works for tables and indexes
//TODO: Table ids can be array of strings (code1)
//TODO: Index keys are an array already. Append unique index table id to the end. (code1)
//TODO: Search table records by partial key (code2). Use prefix search
//TODO: Search indexes by partial key (code2). Use prefix search
