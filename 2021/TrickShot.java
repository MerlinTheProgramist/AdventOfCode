
public class TrickShot {
  
  static public int yPos (int vel){
    return (int)(-1*Math.sqrt(8*vel+1)-1)/2;
  }
  static public int xPos (int vel){
    //if(target_min_x>0)
    return (int)(Math.sqrt(8*vel+1)-1)/2;
    //else
    //  return (int)(-1*Math.sqrt(8*vel+1)+1)/2;
  }
  public static void main(String[] args) {
  
    int target_min_y = 10;
    int target_max_y = 5;

    int target_min_x = 20;
    int target_max_x = 30;

  



    int potential = 0;
    
    int topYvel = ((target_min_y+1)*target_min_y)/2;

    int topXvel = ((target_min_x+1)*target_min_x)/2;

    System.out.println(topYvel);
    while (target_min_y>=target_max_y){
      
      System.out.println(topYvel);
      System.out.println("min_x: "+target_min_y);
      target_min_y = yPos(topYvel);
      topYvel--;

      // int min_x = target_min_x;
      // while (min_x>=target_max_x){
      //   System.out.println(topXvel);
      //   System.out.println("min_x: "+min_x);
      //   min_x = xPos(topXvel);
      //   topXvel--;
      //   potential++;
      // }

    }
    //System.out.println(topYvel);
    System.out.println(potential);
  }
}
// 4095