#include <iostream>
#include <stack>
#include <regex>

inline int ROWS = 8;  
inline int COLS = 9;

using namespace std;

int main()
{

  stack<char> stacks[COLS];

  string str_in[ROWS];
  for(int x=ROWS-1;x>=0;x--)
  {
    getline(cin,str_in[x],'\n');
  }

  for(string s : str_in)
  {
    for(int y=0;y<COLS;y++)
      if(s[y*4+1]!=' ')
      stacks[y].push(s[y*4+1]);
  }

  cin.ignore(256,'\n');
  cin.ignore(256,'\n');
  
  // regex rgx(" [0-9]+");
  while(true)
  {
    string line;
    getline(cin, line,'\n');
    if(line=="") break;
    
    for (int i = 0; i < line.length(); ++i)
    {
        if (isalpha(line[i]))
            line[i] = ' ';
    }    
    //regex_token_iterator<string::iterator> nums(line.begin(),line.end(),rgx);     
    std::stringstream nums(line);
   
    // cout <<line << endl;
     
    int n;// = stoi(*nums++);
    int f;//= stoi(*nums++);
    int t;// = stoi(*nums++);
    nums>> n;
    nums>> f;
    nums>> t;
    
    cout<< n << endl;
    cout<< f << endl;
    cout<< t << endl;
    
    char moving[n];
    for(int i=n-1;i>=0;i--)
    {    
      moving[i] = stacks[f-1].top();      
      stacks[f-1].pop();
    }
    for(char c:moving)
      stacks[t-1].push(c);
      
  }  
  
  
  for(auto& stack : stacks)
  {
    cout << stack.top();
  }
  cout << endl;
}