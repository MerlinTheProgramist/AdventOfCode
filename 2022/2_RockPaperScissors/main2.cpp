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
        
    
    
    if(b==3) //win
      points+=6 + ((a+1>3)?1:(a+1));     
    else if(b==2) // draw
      points+=3 + a;
    else
      points+= ((a-1==0)?3:(a-1));
    cout << points << endl;
  }
    
  cout << points << endl;
}