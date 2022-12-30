#include <iostream>
#include <sstream>
#include <map>
#include <set>

using namespace std;

struct Vector2
{
  int y;
  int x;
  bool operator<(const Vector2& o) const {return y<o.y || (y == o.y && x < o.x);}
  Vector2(int y=0,int x=0):y(y),x(x){}
  Vector2& operator+=(const Vector2& o){
    this->y+=o.y;
    this->x+=o.x;
    return *this;
  }
};


map<char,Vector2> vecs = {
  {'U',{0,-1}},  
  {'D',{0,1}},
  {'L',{-1,0}},
  {'R',{1,0}}
}; 

int main()
{
  set<Vector2> visited;
  
  Vector2 head;
  
  Vector2 tail;  
  visited.insert(tail);
  
  string line;
  while(getline(cin,line))
  {
    stringstream ss(line);
    char dir;
    ss >> dir;
    int units;
    ss >> units;
    
    Vector2 dirV=vecs[dir];
    for(int i=0;i<units;i++)
    {
        if(abs(head.x+dirV.x-tail.x)>1 || abs(head.y+dirV.y-tail.y)>1)
        {
          tail = head;
          visited.insert(tail);
        }
        head+=dirV;
    }
  }
  
  cout << visited.size() << endl;
}
