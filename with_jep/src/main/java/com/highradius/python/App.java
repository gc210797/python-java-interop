package com.highradius.python;

import jep.Interpreter;
import jep.SharedInterpreter;
import jep.python.PyObject;

/**
 * Hello world!
 *
 */
public class App 
{
    public static void main( String[] args )
    {
        try (Interpreter interp = new SharedInterpreter()) {
            interp.exec("from jep_test.example import slice_csv");

            interp.set("url", "https://github.com/cs109/2014_data/blob/master/countries.csv");

            PyObject arr = interp.getValue("slice_csv(url)", PyObject.class);

            System.out.println(arr);
        }
    }
}
