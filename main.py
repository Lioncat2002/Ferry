"""
This file is used for testing the documentation generation
'''doc inside the function can be used for generating docs

Example:
def some_function_name():
    '''doc 
    will generate a doc file  
    This will generate documentation of commented  
    functions as well like this one :p  
    '''
    ...
    code
    ...

"""
def hello():
    '''doc
    # This is a test  
    Prints hello World    
    Adding two or more spaces after a line adds a <br> in the html  
    test  
    >>> Hellow
    '''
    print('Hello World!')

def testing():
    '''doc
    This is another function
    used for testing the doc generation
    '''
    print('Meow')