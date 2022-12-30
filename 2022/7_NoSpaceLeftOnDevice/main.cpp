#include <cstdint>
#include <string>
#include <iostream>
#include <unordered_map>
#include <sstream>
#include <regex>

using namespace std;

inline int TOT_SIZE = 70000000;
inline int UP_SIZE = 30000000;


struct Node{
  string name;  
  Node(string name):name(name){}
};

struct File:Node{
  int size;
  File(string n,int s):Node(n),size(s){}
};

struct Folder:Node{
  Folder* parent; 
  unordered_map<string,Folder*> child_dirs;
  unordered_map<string,File*> child_files;
  Folder(string n, Folder* par):Node(n),parent(par){}
  
  Folder* dir(string n){
    // auto tmp= child_dirs.find(n);
    // if(tmp!=child_dirs.end())
    //   return tmp->second;
    // if node not fould, then create it
    return child_dirs.insert(make_pair(n,new Folder(n,this))).first->second;
  }
  
  Node* file(string n,int s)
  {
    return child_files.insert(make_pair(n,new File(n,s))).first->second;
  }
  
  int getSize(){
    int sum=0;
    for(auto c:child_dirs)
      sum += c.second->getSize();
    for(auto c:child_files)
      sum += c.second->size;
    return sum;
  }
  
  long long sum_traverse()
  {
      long long sum=0;
      for(auto c:child_files)
        sum+=c.second->size;
    
      for(auto c:child_dirs)
        sum+=c.second->sum_traverse();
      return sum;
  }
  
  long long  find_smallest(const long long& min,long long& smallest)
  {
      long long sum=0;
      for(auto c:child_files)
        sum+=c.second->size;
    
      for(auto c:child_dirs)
      {
        sum += c.second->find_smallest(min,smallest);
        // if(sub_sm>=min && sub_sm<smallest)
          // smallest = sub_sm;
      }
      if(sum>=min && sum<smallest)
        smallest = sum;
      
      
      return sum;
  }
};


int main()
{
  Folder* DirTree = new Folder("/",nullptr);
  Folder* currDir = DirTree;

  string line, f;
  
  while(getline(cin,line))
  {
    stringstream ss{line};
    if(line[0]=='$')
    {
      ss >> f >> f;
      if(f=="cd")
      {
        string name;
        ss >> name;
        currDir = (name=="..") ? (currDir->parent) : (currDir->dir(name));
      }
      // LS alone if it would do anything
      // currDir = currDir
      continue;
    }
    
    // LS output processing
    string f;
    string name;
    ss >> f;
    ss >> name;
    if(f=="dir")
    {
      // add Folder to the tree
      currDir->dir(name);
       continue; 
    }
    // add File to the tree
    currDir->file(name,stoi(f));    
  }
  
  // find folders with less than 100000 size
  
  long long sum = DirTree->sum_traverse();
  cout <<"Total usage: " << sum << endl;
  
  const long long minToDelete = UP_SIZE-(TOT_SIZE-sum);
  long long smallest = sum;
  cout <<"Have to delete min: : " << minToDelete << endl;
  DirTree->find_smallest(minToDelete,smallest);
  cout <<"Delete folder with: " << smallest << endl;

}
