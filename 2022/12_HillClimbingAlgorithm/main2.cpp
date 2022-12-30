#include <iostream>
#include <sstream>
#include <algorithm>
#include <vector>
#include <map>
#include <list>
#include <bits/stdc++.h>

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
  Vector2 operator+(Vector2& b){
   return Vector2(y+b.y,x+b.x); 
  }
  bool operator==(const Vector2& b){
    return x==b.x && y==b.y;
  }
  
  operator string() const 
  {
    return to_string(y)+","+to_string(x);
  }
};

const Vector2 DIR[4] = {
  Vector2(-1,0),
  Vector2(1,0),
  Vector2(0,-1),
  Vector2(0,1)
};

struct Graph
{
  const int h_size, w_size;
  map<Vector2,int> mapo;

  Graph(::map<Vector2,int> mapo, int height, int width):h_size(height),w_size(width),mapo(mapo){}
  
   
vector<Vector2> getAdj(Vector2 pos)
{
  vector<Vector2> out;
  
  for(auto dir:DIR)
  {
    Vector2 n_pos = pos+dir;
    if(n_pos.x<0 || n_pos.y<0 || n_pos.x>=w_size || n_pos.y>=h_size)
        continue;
    cout << (string)pos << " > " << (string)n_pos<< endl;
   
    if(mapo[n_pos]>=mapo[pos]-1)
    {
      out.push_back(n_pos);
    }
  }
  return out;
}

// find the shortest route using BFS algorithm
int BFS(const Vector2 src, vector<Vector2>& dest)
{
  
    
  int shortest=INT_MAX;
    
  ::list<Vector2> queue;

  map<Vector2,int> dist; 
  for(int y=0;y<h_size;y++)
    for(int x=0;x<w_size;x++)
      dist[{y,x}] = INT_MAX;
  
  queue.push_back(src);  
  dist[src] = 0;
  // visited.push_back(src);
  
  while(!queue.empty())
  {
    const Vector2 u = queue.front();
    queue.pop_front();
    

    for(auto ad : getAdj(u))
    {

      if(dist[ad]<=dist[u]+1) // skip if its longer path
        continue;

      cout << (string)u << endl;
      
      // visited.push_back(ad);
      dist[ad] = dist[u] + 1;
      queue.push_back(ad);
      
      cout <<"(" << (string)u << ") -> ("<<(string)ad << ")"<< endl;
      cout << dist[ad]  << endl;
      cout << (char)(mapo[ad]+'a') << endl;
      
      
      if(find(dest.begin(),dest.end(),ad)!=dest.end())
        shortest = min(shortest,dist[ad]);
    }
  }
  return shortest;
}

};

int main()
{
  string trs;  
  string line;
  
  map<Vector2,int> map;
  vector<Vector2> starting_points;
  Vector2 end;
  
  
  int w=0;
  int h=0;
  
  // read input
  while(getline(cin,line))
  {
    w=line.length();
    for(int i=0;i<w;i++)
    {
      if(line[i]=='a' || line[i]=='S')
      {
        starting_points.push_back({h,i});
        line[i] = 'a';
      }
      else if(line[i]=='E')
      {
        end = {h,i};
        line[i] = 'z';
      }
      
      
        map[{h,i}] = line[i]-'a';
      
    } 
    h++;        
  }  
  
  
  
  // create and fill graph
  Graph graph(map,h,w);
  
  cout <<"start: " << (string)end << endl;
  for(auto rend : starting_points)
    cout <<"end: " << (string)rend <<endl;
  
  int shortest = graph.BFS(end,starting_points);
  // find shortest path
  cout << "SHORTEST PATH IS: " << shortest << endl;
}
