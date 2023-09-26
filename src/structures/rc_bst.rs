use std::cell::RefCell;
use std::rc::Rc;
use crate::structures::rc_node::{NodeDir, Node, Tree};
use core::fmt;
//use serde::export::Formatter;

/// Public trait that houses methods that are used with Tree classes
#[derive(Debug)]
pub struct BST<T: Clone + ::std::fmt::Debug>  {
    root: NodeDir<T>,
    pub count: u32
}
impl<T: Clone + ::std::fmt::Debug> BST<T>  {
    /// Initialize BST with nothing
    /// the next value added is automatically the root
    pub fn new()->Self{
        BST {
            root: None,
            count: 0
        }
    }
    /// Checks if the BST<T> is empty
    pub fn is_empty(&self)-> bool{self.count == 0}
    /// Creates another node with a key data pair
    fn new_node_dir(&self, key:u32, data: T) -> NodeDir<T>{
        Some(Rc::new(RefCell::new(Node::new(key,data))))
    }
    /// Adds a node with kv pair to the BST
    pub fn add(&mut self, key:u32, data: T){
        if self.is_empty(){
            self.root = self.new_node_dir(key, data);
            self.count = 1;
            return;
        }
        let kv_pair = (key, data);
        let curr = Rc::clone(self.root
            .as_ref().unwrap());
        self.count += 1;
        self.recursive_add(kv_pair, curr);
    }
    ///Helper method for add
    fn recursive_add(&mut self, kv_p:(u32, T), node: Rc<RefCell<Node<T>>>){
        let mut inner_value:
            std::cell::RefMut<Node<T>> = (*node).borrow_mut();

        if &kv_p.0 <= &inner_value.key {
            if let None = inner_value.get_left(){
                inner_value.set_left(self.new_node_dir(kv_p.0,kv_p.1))
            }else {
                let left = Rc::clone(inner_value.left
                    .as_ref().unwrap());
                self.recursive_add(kv_p,left);
            }
            return;
        }
        if &kv_p.0 > &inner_value.key{
            if let None = inner_value.right {
                inner_value.set_right(self.new_node_dir(kv_p.0,kv_p.1))
            }else{
                let right = Rc::clone(inner_value.right
                    .as_ref().unwrap());
                self.recursive_add(kv_p,right);
            }
            return;
        }
    }
    /// Recurse through the right most node ( if there is )
    /// to find the node with the largest node
    fn _max_(&self, node: &NodeDir<T>) ->NodeDir<T>{
        let mut last = node.clone();
        let mut next = node.clone();
        while let Some(node) = next{
            next = node.borrow().right.clone();
            last = Some(node)
        }
        last
    }
    /// Recurse through the left most node ( if there is )
    /// to find the node with the lowest node
    fn _min_(&self, node:&NodeDir<T>)-> NodeDir<T>{
        let mut last = node.clone();
        let mut next = node.clone();
        while let Some(node) = next{
            next = node.borrow().left.clone();
            last = Some(node)
        }
        last
    }
    pub fn min(&self)-> NodeDir<T>{
        return if let None = self.root{
            None
        }
        else {
            self._min_(&self.root)
        }
    }
    pub fn max(&self)-> NodeDir<T>{
        return if let None = self.root{
            None
        }
        else {
            self._max_(&self.root)
        }
    }
    pub fn find(&self, key:u32)->NodeDir<T>{
        return if let None = self.root{
            None
        }
        else {
            self._find(key,&self.root)
        }
    }
    fn _find(&self,key: u32, node:&NodeDir<T>)-> NodeDir<T>{
        let mut last = node.clone();
        let mut next = node.clone();
        while let Some(node) = next{
            next = {
                let node_key = node.borrow().key;
                if &key <= &node_key{
                    node.borrow().left.clone()
                }else {
                    node.borrow().right.clone()
                }
            };
            last = Some(node)
        }
        last
    }
    ///Using pointers, the input array is the output
    pub fn inorder_sort(&self, arr: &mut Vec<T>){
        self.inorder(&self.root,arr)
    }
    ///Helper method for the inorder sort and inorder traversal.
    ///It uses cloning to get the data from the node since its not public
    fn inorder(&self, node: &NodeDir<T>, arr: &mut Vec<T>){
        if let Some(node) = node{
            self.inorder(&node.borrow().left,arr);
            arr.push(node.borrow().get_data());
            self.inorder(&node.borrow().right,arr);
        }
    }
    ///Using pointers, the input array is the output
    /// It uses cloning to get the data from the node since the data isn't public
    pub fn postorder_sort(&self, arr: &mut Vec<T>){
        self.postorder(&self.root, arr)
    }
    ///Helper method for preorder that's recursive. For free would work as well
    fn postorder(&self, node: &NodeDir<T>, arr: &mut Vec<T>){
        if let Some(node) = node{
            self.postorder(&node.borrow().left,arr);
            self.postorder(&node.borrow().right,arr);
            arr.push(node.borrow().get_data());
        }
    }
    pub fn preorder_sort(&self, arr: &mut Vec<T>){
        self.preorder(&self.root, arr)
    }
    fn preorder(&self, node: &NodeDir<T>, arr: &mut Vec<T>){
        if let Some(node) = node{
            arr.push(node.borrow().get_data());
            self.preorder(&node.borrow().left,arr);
            self.preorder(&node.borrow().right,arr);
        }
    }
    pub fn get_left(&self) -> NodeDir<T>{
        self.root.as_ref().unwrap()
            .borrow().left.clone()
    }

    pub fn get_right(&self) -> NodeDir<T>{
        self.root.as_ref().unwrap()
            .borrow().right.clone()
    }
    pub fn is_right(&self)->bool{
        self.root.as_ref().unwrap()
            .borrow().right.is_some()
    }
    pub fn is_left(&self)-> bool{
        self.root.as_ref().unwrap()
            .borrow().left.is_some()
    }
}