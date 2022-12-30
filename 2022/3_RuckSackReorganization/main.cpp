
#include <iostream>

using namespace std;

int main()
{
  int sum = 0;
  
  string line;
  int line_i=0;
  while(std::cin)
  {
  
    getline(std::cin,line);
    if(line.length()==0) break;
    const int n = line.length();
    const int hn = n/2-1;
    int i = -1;
    bool letters[52]{false};
  
    while(i++<hn)
    {
      const char l=line[i];
      if(l<='Z')
        letters[l -'A']=true;
      else
        letters[l -'a'+26]=true;
      cout << l;
      
    }
    cout <<endl;
    i--;
    while(i++<n)
    { 
      const char l=line[i];
      if(l>='A' && l<='Z'&& letters[l -'A'])
      {
          sum+=l-'A'+1;
          // letters[l-'A']=false;
          cout << (char)l << " ";
      }
      else if(l<='z' && l>='a' && letters[l-'a'+26])
      {  
        sum+=l-'a'+1;
        // letters[l-'a'+26]=false;
        cout << (char)l << " ";
      } 
      // cout << l;
      
    }
    cout << endl;
  }
    
  cout << sum << endl;
}