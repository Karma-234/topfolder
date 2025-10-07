pub fn my_struct() {
    struct SuperUser {
        username: String,
        password: String,
        role: String,
        can_delete_posts: bool,
    }
    // To define a function for the struct, you need to implement the `impl` keyword.
    impl SuperUser {
        fn print_info(&self) {
            println!(
                "Username: {}, Password: {}, Role: {}, Can delete posts: {}\\n",
                self.username, self.password, self.role, self.can_delete_posts
            );
        }
    }
    let my_super_user = SuperUser {
        username: String::from("admin"),
        password: String::from("password"),
        role: String::from("admin"),
        can_delete_posts: true,
    };
    my_super_user.print_info();
    // You can create new instaces of a strct if they share the same fields.
    let mut normal_user = SuperUser {
        can_delete_posts: false,
        ..my_super_user
    };
    normal_user.print_info();

    // Structs should be mutable so you can easily reassign values.\\
    normal_user.username = String::from("user");
    normal_user.print_info();
}
