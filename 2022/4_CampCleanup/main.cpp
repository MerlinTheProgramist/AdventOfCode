#include <sstream>
#include <iostream>
#include <regex>

using namespace std;

int main()
{
  int sum=0;
  string line;
  
  regex rgx("");
  stringstream sti(line);
  while(std::cin)
  {
    getline(std::cin,line,',');
    if(line.length()==0) break; 
    
    sti.clear();
    sti.str(line);
    int a,b;
    sti >> a;
    sti >> b;
    b=abs(b);
    
    getline(std::cin,line,'\n');
    sti.clear();
    sti.str(line);
    int x,y;
    sti >> x;
    sti >> y;
    y=abs(y);
    
    cout << a <<" "<< b <<" "<< x <<" "<< y << " ";
    
    if(a<=y && b>=x)
    {sum++; cout <<sum<< "+";}
    cout << endl;
  }
  cout << sum << endl;
  
}