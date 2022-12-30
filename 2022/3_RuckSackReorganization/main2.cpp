
#include <iostream>
#include <algorithm>
using namespace std;

int main()
{
  int sum = 0;
  
  string line;
  int line_i=1;
  while(std::cin)
  {
  
    int letters[52]{false};
    getline(std::cin,line);
    if(line.length()==0) break;

    if(line_i%4==0)
    {
      fill(begin(letters),begin(letters)+52,false);
      line_i=1;
      for(int i=0;i<52;i++)
        if(letters[i]==3)
          sum+=i+1;
    }
    for(char l : line)
    {   
    
      if(l<='Z' && letters[l-'A']!=line_i)
          letters[l -'A']++;
 else if(l>='a' && letters[l-'a'+26]!=line_i)
          letters[l -'a'+26]++;
    }
    
    cout << sum << endl;
    
    line_i++;
  }
    
  cout << sum << endl;
}