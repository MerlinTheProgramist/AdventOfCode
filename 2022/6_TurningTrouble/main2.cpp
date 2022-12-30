/*
  finding first 4 char substring in string that have no repeats
*/
#include <string>
#include <iostream>
#include <set>

using namespace std;


// 4 for part one, 14 for part two
inline int search_len=14;

int main()
{
  string str;
  getline(cin,str);
  int last_doubled=search_len-1;
  
  for(int i=search_len;i<str.length();i++)
  {
    const char curr_c = str[i];
    
    cout <<i<< " "<< last_doubled << endl <<endl;
    
    set<char> substr;
    for(int subi=i;subi>i-search_len;subi--)
    {
      substr.insert(str[subi]);
    }
    if(substr.size()==search_len)
    {
      cout << i+1 << endl;
    return 0;
    }
        
  }
}