import java.io.*;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReadWriteLock;
import java.util.concurrent.locks.ReentrantReadWriteLock;

public class Main
{
	public static void main(String[] args) {
		  final ReadWriteLock readWriteLock = new ReentrantReadWriteLock();
   final Lock writeLock = readWriteLock.writeLock();
   final Lock readLock = readWriteLock.readLock();
 final List<String> list = new ArrayList<>();
        // acquire the thread for writing
        writeLock.lock();
        try {
          
            list.add("This should be allowed");
            
        }
        finally {
            // release the lock so that other read threads are read concurrently without blocking
            writeLock.unlock();
        }
        // acquire the thread for reading
        readLock.lock();
        try {
            System.out.println(list); //Still not modified
          //I do it on a purpose for this blog post but what about when accidently perform this operation
            list.add("This should't be allowed when reading");
//            System.out.println(list);
          //In java ,even though we have read_lock it doesn't mean we don't have permission to write it. 
        }
        finally {
            // Explicitly unlock otherwise undefined behaviour
            readLock.unlock();
        }
    //It's supposed throw exceptions since we write one data to the empty list when we have writelock    
        System.out.println(list.get(1));            
	}
}
  
