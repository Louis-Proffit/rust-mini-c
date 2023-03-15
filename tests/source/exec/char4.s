	.text
	.globl main
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, %rax
	movq %rsi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	addq -16(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, %rdi
	movq $0, %rsi
	call f
	movq %rax, %rdi
	addq $0, %rsp
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rsi
	call f
	addq $0, %rsp
	movq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $2, %rsi
	call f
	movq %rax, %rdi
	addq $0, %rsp
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $3, %rsi
	call f
	movq %rax, -8(%rbp)
	addq $0, %rsp
	movq -8(%rbp), %r9
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rdi
	addq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

