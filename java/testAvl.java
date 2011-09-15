// This software is in the public domain, furnished "as is", without technical
// support, and with no warranty, express or implied, as to its usefulness for
// any purpose.
import java.util.*;

class TestAVL {
  static Random generator = new Random(123456);

  public static AVL randomTree(int nbElements) {
    AVL myTree = new AVL();
    List<Integer> content = new LinkedList<Integer>();
    for(int i = 0; i < nbElements; i++) {
      int v = generator.nextInt(1000);
      content.add(v);
      myTree.add(v);
    }
    boolean check = true;
    for(int i: content) {
      check = check && myTree.hasKey(i);
    }
    System.out.printf("Created tree with %d elements, ok: %s\n", nbElements, check);
    return myTree;
  }

  public static void main(String[] args) {
    System.out.println("AVL test:");
    {
      AVL myTree = new AVL();
      for(int i = 0; i < 10; i++)
        myTree.add(i);
      myTree.print();
      System.out.printf("Height:%d %s %s\n", myTree.getHeight(), myTree.checkBalanced(), myTree.checkTree());
      myTree.remove(5);
      myTree.remove(7);
      myTree.print();
    }
    for(int i = 0; i < 10; i++) {
      AVL myTree = randomTree(500);
      System.out.printf("Height:%d %s %s\n", myTree.getHeight(), myTree.checkBalanced(), myTree.checkTree());
      }
    }
}

