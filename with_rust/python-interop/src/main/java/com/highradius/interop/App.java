package com.highradius.interop;

import java.util.List;

/**
 * Hello world!
 *
 */
public class App 
{
    private static native List<String> readCsv(String csv);

    static {
        System.loadLibrary("python_interop");
    }

    public static void main( String[] args )
    {
        System.out.println(readCsv("test"));
    }
}
