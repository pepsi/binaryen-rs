(
    (functions (
        (name . "example")
        (params (
            (name . "lhs")
                (type . I32))
            ((name . "rhs")
                (type . I32))) 

        (instructions 
            (Add 
                (Local . "lhs") 
                (Local . "rhs"))
            (Add 
                (Local . "lhs") 
                (Local . "rhs"))
            )
        )
    )
)
