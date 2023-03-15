	.text
	.globl main
fact_imp:
	pushq %rbp
	movq %rsp, %rbp
	addq $-64, %rsp
	movq %rdi, -8(%rbp)
	movq $1, %rax
	movq %rax, -64(%rbp)
	movq -8(%rbp), %r10
L2131:
	movq %r10, -48(%rbp)
	movq $1, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r10
	cmpq %r10, %r10
	setg %r10b
	movq %r10, -48(%rbp)
	movq -48(%rbp), %r10
	testq %r10, %r10
	jz L2117
	movq $1, -40(%rbp)
	movq -40(%rbp), %r10
	subq %r10, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq $1, -32(%rbp)
	movq -32(%rbp), %r10
	addq %r10, -24(%rbp)
	imulq -24(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp L2131
L2117:
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	movq $0, %rdi
	call fact_imp
	addq $0, %rsp
	movq $1, %rsi
	cmpq %rsi, %rax
	sete %al
	testq %rax, %rax
	jnz L2110
L2108:
	movq $1, %rdi
	call fact_imp
	addq $0, %rsp
	movq $1, %rdx
	cmpq %rdx, %rax
	sete %al
	testq %rax, %rax
	jnz L2103
L2101:
	movq $5, %rdi
	call fact_imp
	addq $0, %rsp
	movq $120, %r8
	cmpq %r8, %rax
	sete %al
	testq %rax, %rax
	jnz L2096
L2094:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
L2096:
	movq $51, %rdi
	call putchar
	addq $0, %rsp
	jmp L2094
L2103:
	movq $50, %rdi
	call putchar
	addq $0, %rsp
	jmp L2101
L2110:
	movq $49, %rdi
	call putchar
	addq $0, %rsp
	jmp L2108
	.data

