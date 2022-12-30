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
  cout << H << ' '<< W << endl;
  
  int max_vis;
  for(int y=1;y<H-1;y++)
  {
    for(int x=1;x<W-1;x++)
    {
        const int t = grid[{y,x}];
        
        int Pvis=0;
        for(int i=x+1;i<W;i++)
        {
          Pvis++;
          if(grid[{y,i}]>=t)
            break;
        }
        
        int vis=0;
        for(int i=x-1;i>=0;i--)
        {
          vis++;
          if(grid[{y,i}]>=t)
            break;
        }
        Pvis*=vis;
        vis=0;
        for(int i=y+1;i<H;i++)
        {
          vis++;
          if(grid[{i,x}]>=t)
            break;
        }
        Pvis*=vis;
        vis=0;
        for(int i=y-1;i>=0;i--)
        {
          vis++;
          if(grid[{i,x}]>=t)
            break;
        }
        Pvis*=vis;
        if(Pvis>max_vis)
          max_vis=Pvis;
    }
  }
    
  cout << max_vis << endl;
}