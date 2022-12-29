package main

import (
	"context"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "github.com/edenreich/payments/codegen"
)

func main() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewBitcoinClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	response, _ := client.SendPayment(ctx, &pb.BtcPaymentRequest{
		FromAddress: "mwSmXgUGd78mUup5332qCyDUy32LoGCAfJ",
		ToAddress:   "myPrkbuYHZNGB8CPnZbDNzp945SLcjPK6z",
		Amount:      100,
	})

	log.Printf("Sent payment %v", response)
}
