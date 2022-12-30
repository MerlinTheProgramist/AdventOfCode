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
  Vector2 operator+(const Vector2& b){
   return Vector2(y+b.y,x+b.x); 
  }
  bool operator==(const Vector2& b){
    return x==b.x && y==b.y;
  }
  
  operator string()
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
  
   
vector<Vector2> getAdj(Vector2& pos)
{
  vector<Vector2> out;
  
  for(auto dir:DIR)
  {
    auto n_pos = pos+dir;
    if(n_pos.x<0 || n_pos.y<0 || n_pos.x>=w_size || n_pos.y>=h_size)
        continue;
   
    if(mapo[n_pos]<=mapo[pos]+1)
    {
      out.push_back(n_pos);
    }
  }
  return out;
}

// find the shortest route using BFS algorithm
bool BFS(Vector2& src, Vector2& dest, ::map<Vector2,int>& dist)
{
  ::list<Vector2> queue;
  // vector<Vector2> visited;
  for(int y=0;y<h_size;y++)
    for(int x=0;x<w_size;x++)
      dist[{y,x}] = INT_MAX;
  
  queue.push_back(src);  
  dist[src] = 0;
  // visited.push_back(src);
  
  while(!queue.empty())
  {
    Vector2& u = queue.front();
    queue.pop_front();
    

    for(auto ad : getAdj(u))
    {

      if(dist[ad]<dist[u]+1 || dist[ad]==dist[u]+1) // skip if its longer path
        continue;
      cout << (string)u << endl;
      
      // visited.push_back(ad);
      dist[ad] = dist[u] + 1;
      queue.push_back(ad);
      
      cout <<"(" << (string)u << ") -> ("<<(string)ad << ")"<< endl;
      cout << dist[ad]  << endl;
      
      if(ad == dest)
        return true;
    }
  }
  return false;
}

};

int main()
{
  string trs;  
  string line;
  
  map<Vector2,int> map;
  Vector2 start, end;
  
  
  int w=0;
  int h=0;
  
  // read input
  while(getline(cin,line))
  {
    w=line.length();
    for(int i=0;i<w;i++)
    {
      if(line[i]=='S')
      {
        start = {h,i};
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
  
  // find shortest path
  ::map<Vector2,int> dist;
  cout << graph.BFS(start,end,dist) << endl;
      
  cout << "start: " << (string)start << endl;
  cout << "end: " << (string)end << endl;
  ::map<Vector2,Vector2> pred;
  cout <<"SHORTEST PATH IS: " << dist[end] <<  endl; 
}
