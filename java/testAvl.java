// This software is in the public domain, furnished "as is", without technical
// support, and with no warranty, express or implied, as to its usefulness for
// any purpose.
import java.util.Random;

class TestAVL {
  static Random generator = new Random(123456);

  public static AVL randomTree(int nbElements) {
    AVL myTree = new AVL();
    for(int i = 0; i < nbElements; i++)
      myTree.add(generator.nextInt(1000));
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
    }
    for(int i = 0; i < 10; i++) {
      AVL myTree = randomTree(1000);
      System.out.printf("Height:%d %s %s\n", myTree.getHeight(), myTree.checkBalanced(), myTree.checkTree());
      }
    }
}

