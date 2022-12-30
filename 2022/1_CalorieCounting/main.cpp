#include <iostream>

using namespace std;

int main()
{

  int sum = 0;
  int max = 0;
  
  string line;
  while(std::cin)
  {
    getline(std::cin,line);
    
    if(line=="")
    {
      if(sum>max)
        max = sum;
      sum=0;
      continue;
    }
    sum += stoi(line);
    
  }
  
  if(sum>max)
    max = sum;
  sum=0;
  
  cout << max << endl;
}