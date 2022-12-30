/*
  finding first 4 char substring in string that have no repeats
*/
#include <string>
#include <iostream>

using namespace std;

// 4 for part one, 14 for part two
inline int search_len=14;

int main()
{
  string str;
  getline(cin,str);
  int last_doubled=search_len-1;
  
  for(int i=1;i<str.length();i++)
  {
    const char curr_c = str[i];
    
    cout <<i<< " "<< last_doubled << endl <<endl;
    
    bool found=false;
    // itherate from current i back to first character of the substring
    for(int sub_ri=i-1;sub_ri>=max(0,i-search_len+1);sub_ri--)
    {
      cout<<sub_ri << " "<<str[sub_ri] << '=' <<curr_c<<endl;
      // if found same char 
      if(curr_c==str[sub_ri])
      {
        // i = sub_ri+search_len;
        last_doubled = max(last_doubled,sub_ri);
        found = true;
        break;
      }
    }
    // if not found same char as curr_c
    // it means we found the substring
    if(!found && i>=last_doubled+search_len)
    {
      cout<<i+1<<endl;
      return 0;
    }
  }
}