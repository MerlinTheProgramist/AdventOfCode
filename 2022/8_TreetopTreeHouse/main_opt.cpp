#include <iostream>
#include <map>
using namespace std;

int W=99,H=99;

struct Vector2
{
  int y;
  int x;
  bool operator<(const Vector2& o) const {return y<o.y || (y == o.y && x < o.x);}
  Vector2(int y,int x):y(y),x(x){}
};


int main()
{
  map<Vector2,int> grid;
  
  string line;
  int h=0;
  while(getline(std::cin,line))
  {
    
    cout << line << endl;
    W=line.length(); 
    int w=0;
    for(char c:line)
      grid.insert(make_pair(Vector2(h,w++),c-'0'));
    h++;
  }
  H = h;
  // cout << H << ' '<< W << endl;
  cout << endl;
  
  // precomputinting max highest tries in each direction
  map<Vector2,int> up;
  map<Vector2,int> down;
  map<Vector2,int> left;
  map<Vector2,int> right;
  
  for(int y=0;y<H;y++)
  {
    for(int x=0;x<W;x++)
    {
      const int x_ = W-x-1;
      const int y_ = H-y-1;
       
      if(y==0 || grid[{y,x}]>up[{y-1,x}])
        up[{y,x}] = grid[{y,x}];
      else
        up[{y,x}] = up[{y-1,x}]; 
      
      if(y_==H-1 || grid[{y_,x}]>down[{y_+1,x}])
        down[{y_,x}] = grid[{y_,x}];
      else
        down[{y_,x}] = down[{y_+1,x}];
      
      if(x==0 || grid[{y,x}]>left[{y,x-1}])
        left[{y,x}] = grid[{y,x}];
      else
        left[{y,x}] = left[{y,x-1}];
      
      if(x_==W-1 || grid[{y,x_}]>right[{y,x_+1}])
        right[{y,x_}] = grid[{y,x_}];
      else 
        right[{y,x_}] = right[{y,x_+1}];
      
      cout << right[{y,x_}];
    }
    cout << endl;
  }

  int vis=2*(W+H-2);

  for(int y=1;y<H-1;y++)
  {
    for(int x=1;x<W-1;x++)
    {
        const Vector2 v = {y,x};
        const int t = grid[v];
      
        if(
          up[{y-1,x}]<t ||  
          down[{y+1,x}]<t ||
          left[{y,x-1}]<t ||
          right[{y,x+1}]<t
        )
        vis++;
    }
  }
    
  cout << vis << endl;
}