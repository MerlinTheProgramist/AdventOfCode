#include <iostream>

using namespace std;

int main()
{
  int points = 0;
  
  string line;
  while(std::cin)
  {
    getline(std::cin,line);
    if(line.length()<3) break;
    int a = line[0]-'A'+1;
    int b = line[2]-'X'+1;
        
    if((b==1 && a==3) || b-1==a) //win
      points+=6;     
    else if(b==a) // draw
      points+=3;
  
    points+=b;  
  }
    
  cout << points << endl;
}